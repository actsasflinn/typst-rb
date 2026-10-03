use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;
use std::ops::Range;
use once_cell::sync::Lazy;

use magnus::{Error, scan_args::{get_kwargs, scan_args}, Ruby, RHash};

use crate::nogvl::without_gvl;

use chrono::{DateTime, Datelike, FixedOffset, Local};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::term::{self, termcolor};
use ecow::eco_format;

use typst::diag::{Severity, SourceDiagnostic, FileResult, Warned};
use typst::foundations::{Bytes, Datetime, Dict, Duration, Value};
use typst::syntax::{DiagSpan, DiagSpanKind, FileId, Lines, Source, VirtualRoot};
use typst::text::{Font, FontBook, FontInfo};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};
use typst_pdf::PdfStandard;
use typst_html::HtmlDocument;
use typst_layout::PagedDocument;

use crate::query::{query, QueryCommand, SerializationFormat};
use crate::formats::pdf::PdfDocument;
use crate::formats::html::HtmlExperimentalDocument;
use crate::formats::svg::SvgDocument;
use crate::formats::png::PngDocument;

use typst_kit::fonts::{self, FontPath, FontStore};
use typst_kit::packages::{FsPackages, SystemPackages, UniversePackages}; //, PackageSpec, PackageVersion};

type CodespanResult<T> = Result<T, CodespanError>;
type CodespanError = codespan_reporting::files::Error;

static LOCAL_FONT_CACHE: Lazy<Mutex<Vec<Font>>> = Lazy::new(|| Mutex::new(Vec::new()));
static FILE_CACHE: Lazy<Mutex<HashMap<String, Arc<FileCache>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

struct FileCache {
    bytes: Bytes,
    source: Mutex<Option<Source>>,
}

#[magnus::wrap(class = "Typst::VirtualWorld")]
pub struct VirtualWorld {
    library: LazyHash<Library>,
    fonts: Arc<FontStore>,
    package_storage: SystemPackages,
    source: Source,
    now: OnceLock<DateTime<Local>>,
}

impl VirtualWorld {
    pub fn new(
        text: String,
        fonts: Option<Vec<Vec<u8>>>,
        font_paths: Option<Vec<PathBuf>>,
        system_fonts: Option<bool>,
        embedded_fonts: Option<bool>,
        local_fonts: Option<bool>,
        package_path: Option<PathBuf>,
        package_cache_path: Option<PathBuf>,
        files: Option<HashMap<String, Vec<u8>>>,
        sys_inputs: Option<HashMap<String, String>>,
    ) -> Self {
        let mut font_store = FontStore::new();
        if system_fonts.unwrap_or(false) {
            let system = cached(
                |cache| &mut cache.system,
                || fonts::system().map(|(path, info)| (path.path, path.index, info)).collect(),
            );
            font_store.extend(system.iter().map(|(path, index, info)| {
                (FontPath { path: path.clone(), index: *index }, info.clone())
            }));
        }
        if embedded_fonts.unwrap_or(false) {
            let embedded = cached(|cache| &mut cache.embedded, || fonts::embedded().collect());
            font_store.extend(embedded.iter().cloned());
        }
        if local_fonts.unwrap_or(false) {
            for font in LOCAL_FONT_CACHE.lock().unwrap().iter() {
                let info = font.info().clone();
                font_store.push((font.clone(), info));
            }
        }
        for path in font_paths.unwrap_or_default() {
            font_store.extend(fonts::scan(&path));
        }
        for data in fonts.unwrap_or_default().iter() {
            let bytes = Bytes::new(data.to_vec());
            if let Some(font) = Font::iter(bytes).next() {
                let info = font.info().clone();
                font_store.push((font, info));
            }
        }

        let package_storage = system_packages(package_path, package_cache_path);

        let mut file_holder = FILE_CACHE.lock().unwrap();
        for (path, data) in files.unwrap_or_default().iter() {
            let clean_path = clean_path(path.as_str());
            file_holder.insert(clean_path, Arc::new(FileCache {
                bytes: Bytes::new(data.to_vec()),
                source: Mutex::new(None),
            }));        
        }

        let inputs = Dict::from_iter(
            sys_inputs.unwrap_or_default()
                .into_iter()
                .map(|(k, v)| (k.into(), Value::Str(v.into()))),
        );

        let features = vec![typst::Feature::Html].into_iter().collect();

        let library = LazyHash::new(Library::builder()
            .with_inputs(inputs)
            .with_features(features).build());

        Self {
            library: library,
            fonts: Arc::new(font_store),
            package_storage: package_storage,
            source: Source::detached(text),
            now: OnceLock::new()
        }
    }
    pub fn new_ruby(
        args: &[magnus::Value],
    ) -> Result<Self, Error> {
        let args = scan_args::<_, (), (), (), _, ()>(args).unwrap();
        let (text,): (String,) = args.required;
        let kw = get_kwargs::<_, (), (
            Option<Vec<Vec<u8>>>,
            Option<Vec<PathBuf>>,
            Option<bool>,
            Option<bool>,
            Option<bool>,
            Option<PathBuf>,
            Option<PathBuf>,
            Option<HashMap<String, Vec<u8>>>,
            Option<RHash>,
        ), ()>(args.keywords, &[], &["fonts", "font_paths", "system_fonts", "embedded_fonts", "local_fonts", "package_path", "package_cache_path", "files", "sys_inputs"]);

        match kw {
            Ok(keywords) => {
                let (
                    fonts,
                    font_paths,
                    system_fonts,
                    embedded_fonts,
                    local_fonts,
                    package_path,
                    package_cache_path,
                    files,
                    sys_inputs,
                ) = keywords.optional;
                
                //let si:HashMap<String, String> = HashMap::default();
                let si = sys_inputs.unwrap().to_hash_map::<String, String>();
                // sys_inputs.unwrap().foreach(|key: String, value: String| {
                //     // Do something with key and value
                //     // Return ForEach::Continue, ForEach::Stop, or ForEach::Delete
                    
                //     Ok(ForEach::Continue)
                // })?;

                let world = without_gvl(move || -> Self {
                    Self::new(text, fonts, font_paths, system_fonts, embedded_fonts, local_fonts, package_path, package_cache_path, files, Some(si.unwrap()))
                });
                Ok(world)
            },
            Err(err) => Err(Error::new(Ruby::get().unwrap().exception_arg_error(), err.to_string()))
        }
    }
    fn range(&self, span: impl Into<DiagSpan>) -> Option<Range<usize>> {
        match span.into().get() {
            DiagSpanKind::Detached => None,
            DiagSpanKind::Number { id, num, sub_range } => {
                self.source(id).ok()?.range(num, sub_range)
            }
            DiagSpanKind::Range { id: _, range } => Some(range),
        }
    }
    pub fn with_inputs(&self, sys_inputs: Option<HashMap<String, String>>) -> Self {
        let features = vec![typst::Feature::Html].into_iter().collect();

        let package_storage = SystemPackages::from_parts(
            Some(self.package_storage.data().unwrap().clone()),
            Some(self.package_storage.cache().unwrap().clone()),
            UniversePackages::new(crate::download::downloader()),
        );

        let inputs = Dict::from_iter(
            sys_inputs.unwrap_or_default()
                .into_iter()
                .map(|(k, v)| (k.into(), Value::Str(v.into()))),
        );

        let library = LazyHash::new(Library::builder()
        .with_inputs(inputs)
        .with_features(features).build());

        Self {
            library: library,
            fonts: self.fonts.clone(),
            package_storage: package_storage,
            source: self.source.clone(),
            now: OnceLock::new()
        }
    }
    fn lookup(&self, id: FileId) -> Lines<String> {
        let src = self.source(id).unwrap();
        return src.lines().clone();
    }
    pub fn to_pdf_ruby(
        &self,
        args: &[magnus::Value],
    ) -> Result<PdfDocument, Error> {
        let args = scan_args::<(), (), (), (), _, ()>(args)?;
        let kw = get_kwargs::<_, (), (Option<Vec<String>>, Option<bool>), ()>(args.keywords, &[], &["pdf_standards","pretty"])?;
        let (pdf_standards, pretty,) = kw.optional;
        let result = without_gvl(move || -> Result<PdfDocument, String> {
            self.to_pdf(pdf_standards, pretty)
        });
        match result {
            Ok(doc) => Ok(doc),
            Err(err) => {
                Err(Error::new(Ruby::get().unwrap().exception_arg_error(), err.to_string()))
            }
        }
    }
    pub fn to_pdf(
        &self,
        pdf_standards: Option<Vec<String>>,
        pretty: Option<bool>
    ) -> Result<PdfDocument, String> {
        let pdf_standards_lookup: HashMap::<&str, PdfStandard> = HashMap::from([
            ("1.4", PdfStandard::V_1_4),
            ("1.5", PdfStandard::V_1_5),
            ("1.6", PdfStandard::V_1_6),
            ("1.7", PdfStandard::V_1_7),
            ("2.0", PdfStandard::V_2_0),
            ("a-1a", PdfStandard::A_1a),
            ("a-1b", PdfStandard::A_1b),
            ("a-2a", PdfStandard::A_2a),
            ("a-2b", PdfStandard::A_2b),
            ("a-2u", PdfStandard::A_2u),
            ("a-3a", PdfStandard::A_3a),
            ("a-3b", PdfStandard::A_3b),
            ("a-3u", PdfStandard::A_3u),
            ("a-4", PdfStandard::A_4),
            ("a-4e", PdfStandard::A_4e),
            ("a-4f", PdfStandard::A_4f),
            ("ua-1", PdfStandard::Ua_1),
        ]);
    
        let mut pdf_standards_vec = Vec::<PdfStandard>::new();
        for pdf_standard in pdf_standards.unwrap_or_else(|| Vec::default()).iter() {
            let result = pdf_standards_lookup.get(pdf_standard.as_str());
            if let Some(value) = result {
                pdf_standards_vec.push(*value);
            } else {
                return Err("error".to_string());
            }
        }
        let standards = typst_pdf::PdfStandards::new(&pdf_standards_vec).unwrap();

        let Warned { output, warnings } = typst::compile::<PagedDocument>(self);

        match output {
            Ok(doc) => {
                Ok(PdfDocument::new(typst_pdf::pdf(&doc, &typst_pdf::PdfOptions {
                    ident: typst::foundations::Smart::Auto,
                    standards,
                    pretty: pretty.unwrap_or_else(|| false),
                    ..Default::default()
                }).unwrap(), format_warnings(self, &warnings)))
            },
            Err(errors) => Err(format_diagnostics(self, &errors, &warnings).unwrap().to_string())
        }
    }
    pub fn to_html_ruby(
        &self,
        args: &[magnus::Value],
    ) -> Result<HtmlExperimentalDocument, Error> {
        let args = scan_args::<(), (), (), (), _, ()>(args)?;
        let kw = get_kwargs::<_, (), (Option<bool>,), ()>(args.keywords, &[], &["pretty"])?;
        let (pretty,) = kw.optional;

        let result = without_gvl(move || -> Result<HtmlExperimentalDocument, String> {
            self.to_html(pretty)
        });
        match result {
            Ok(doc) => Ok(doc),
            Err(err) => Err(Error::new(Ruby::get().unwrap().exception_arg_error(), err.to_string()))
        }
    }
    pub fn to_html(
        &self,
        pretty: Option<bool>,
    ) -> Result<HtmlExperimentalDocument, String> {
        let Warned { output, warnings } = typst::compile::<HtmlDocument>(self);

        match output {
            Ok(doc) => {
                let buffer = typst_html::html(&doc, &typst_html::HtmlOptions { pretty: pretty.unwrap_or(false) }).unwrap();
                Ok(HtmlExperimentalDocument::new(buffer, format_warnings(self, &warnings)))
            },
            Err(errors) => Err(format_diagnostics(self, &errors, &warnings).unwrap().to_string())
        }
    }
    pub fn to_svg_ruby(
        &self,
        args: &[magnus::Value],
    ) -> Result<SvgDocument, Error> {
        let args = scan_args::<(), (), (), (), _, ()>(args)?;
        let kw = get_kwargs::<_, (), (Option<bool>,Option<bool>,), ()>(args.keywords, &[], &["pretty", "render_bleed"])?;
        let (pretty, render_bleed,) = kw.optional;

        let result = without_gvl(move || -> Result<SvgDocument, String> {
            self.to_svg(pretty, render_bleed)
        });
        match result {
            Ok(doc) => Ok(doc),
            Err(err) => Err(Error::new(Ruby::get().unwrap().exception_arg_error(), err.to_string()))
        }
    }
    pub fn to_svg(
        &self,
        pretty: Option<bool>,
        render_bleed: Option<bool>,
    ) -> Result<SvgDocument, String> {
        let Warned { output, warnings } = typst::compile::<PagedDocument>(self);

        match output {
            Ok(doc) => {
                Ok(SvgDocument::new(
                    doc
                    .pages()
                    .iter()
                    .map(|page|
                        typst_svg::svg(
                            page, &typst_svg::SvgOptions{
                                render_bleed: render_bleed.unwrap_or(false),
                                pretty: pretty.unwrap_or(false)
                            }
                        )
                    )
                    .collect(), format_warnings(self, &warnings)))
            },
            Err(errors) => Err(format_diagnostics(self, &errors, &warnings).unwrap().to_string())
        }    
    }
    pub fn to_png_ruby(
        &self,
        args: &[magnus::Value],
    ) -> Result<PngDocument, Error> {
        let args = scan_args::<(), (), (), (), _, ()>(args)?;
        let kw = get_kwargs::<_, (), (Option<bool>, Option<f32>,), ()>(args.keywords, &[], &["render_bleed", "ppi"])?;
        let (render_bleed, ppi) = kw.optional;

        let result = without_gvl(move || -> Result<PngDocument, String> {
            self.to_png(render_bleed, ppi)
        });
        match result {
            Ok(doc) => Ok(doc),
            Err(err) => Err(Error::new(Ruby::get().unwrap().exception_arg_error(), err.to_string()))
        }
    }
    pub fn to_png(
        &self,
        render_bleed: Option<bool>,
        ppi: Option<f32>,
    ) -> Result<PngDocument, String> {
        let Warned { output, warnings } = typst::compile::<PagedDocument>(self);

        match output {
            Ok(doc) => {
                Ok(PngDocument::new(
                    doc
                    .pages()
                    .iter()
                    .map(|page|
                        typst_render::render(page,
                            &typst_render::RenderOptions {
                                pixel_per_pt: typst::utils::Scalar::new(f64::from(ppi.unwrap_or(144.0) / 72.0)),
                                render_bleed: render_bleed.unwrap_or(false),
                            },
                        ).encode_png().unwrap()
                    )
                    .collect(), format_warnings(self, &warnings)))
            },
            Err(errors) => Err(format_diagnostics(self, &errors, &warnings).unwrap().to_string())
        }
    }
    pub fn info(&self) -> (String, String) {
        let font_book = format!("Font Book: {:#?}", &self.fonts.book());
        let filename = FILE_CACHE.lock().unwrap().iter()
            .map(|(filename, _file)| format!("Filename: {}", filename) ).collect();
        (font_book, filename)
    }
    pub fn query(
        &self,
        selector: String,
        field: Option<String>,
        one: bool,
        format: Option<String>,
    ) -> Result<String, Error> {
        let format = match format.unwrap().to_ascii_lowercase().as_str() {
            "json" => SerializationFormat::Json,
            "yaml" => SerializationFormat::Yaml,
            _ => return Err(Error::new(Ruby::get().unwrap().exception_arg_error(), "unsupported serialization format".to_string()))
        };
    
        let result = query(
            &self,
            &QueryCommand {
                selector: selector.into(),
                field: field,
                one,
                format,
            },
        );
    
        match result {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::new(Ruby::get().unwrap().exception_arg_error(), error.to_string()))
        }
    }
}

impl World for VirtualWorld {
    fn library(&self) -> &LazyHash<Library> { &self.library }
    fn book(&self) -> &LazyHash<FontBook> { &self.fonts.book() }
    fn main(&self) -> FileId { self.source.id() }

    fn source(&self, id: FileId) -> FileResult<Source> { 
        if id == self.source.id() { return Ok(self.source.clone()); }

        match id.root() {
            VirtualRoot::Package(pkg) => {
                match self.package_storage.obtain(pkg) {
                    Ok(package) => {
                        match id.vpath().realize(package.path()) {
                            Ok(file_path) => {
                                match fs::read(file_path) {
                                    Ok(entry) => {
                                        let text = std::str::from_utf8(&entry)
                                        .map_err(|_| typst::diag::FileError::InvalidUtf8)?
                                        .trim_start_matches('\u{feff}');

                                        let source = Source::new(id, text.to_string());
                                        Ok(source)
                                    },
                                    Err(_) => Err(typst::diag::FileError::NotFound(PathBuf::from(id.vpath().get_without_slash())))
                                }
                            },
                            Err(_) => Err(typst::diag::FileError::NotFound(PathBuf::from(id.vpath().get_without_slash())))
                        }
                    },
                    Err(_) => Err(typst::diag::FileError::NotFound(PathBuf::from(id.vpath().get_without_slash())))
                }
            },
            VirtualRoot::Project => {
                let key = get_key(id);
                let files = FILE_CACHE.lock().unwrap();
        
                if let Some(entry) = files.get(&key) {
                    let mut source_cache = entry.source.lock().unwrap();
                    if let Some(source) = &*source_cache {
                        return Ok(source.clone());
                    }
        
                    let text = std::str::from_utf8(&entry.bytes)
                        .map_err(|_| typst::diag::FileError::InvalidUtf8)?
                        .trim_start_matches('\u{feff}');
        
                    let source = Source::new(id, text.to_string());
                    *source_cache = Some(source.clone());
                    Ok(source)
                } else {
                    Err(typst::diag::FileError::NotFound(PathBuf::from(id.vpath().get_without_slash())))
                }
        
            }
        }
    }

    /// Attempt to load a file from cache or a package file from the package storage
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        match id.root() {
            VirtualRoot::Package(pkg) => {
                match self.package_storage.obtain(pkg) {
                    Ok(package) => {
                        match id.vpath().realize(package.path()) {
                            Ok(file_path) => {
                                // println!("file Package Path Rooted: {:#?}", file_path);
                                match fs::read(file_path) {
                                    Ok(entry) => Ok(Bytes::new(entry.to_vec())),
                                    Err(_) => Err(typst::diag::FileError::NotFound(PathBuf::from(id.vpath().get_without_slash())))
                                }
                            },
                            Err(_) => Err(typst::diag::FileError::NotFound(PathBuf::from(id.vpath().get_without_slash())))
                        }
                    },
                    Err(_) => Err(typst::diag::FileError::NotFound(PathBuf::from(id.vpath().get_without_slash())))
                }
            },
            VirtualRoot::Project => {
                let path = get_key(id);
                let files = FILE_CACHE.lock().unwrap();
                if let Some(entry) = files.get(&path) {
                    Ok(entry.bytes.clone())
                } else {
                    Err(typst::diag::FileError::NotFound(PathBuf::from(id.vpath().get_without_slash())))
                }
            }
        }
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.font(index)
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        let now = self.now.get_or_init(chrono::Local::now);

        let now = match offset {
            None => now.fixed_offset(),
            Some(offset) => {
                let seconds = offset.seconds().trunc();
                if !seconds.is_finite()
                    || seconds < f64::from(i32::MIN)
                    || seconds > f64::from(i32::MAX)
                {
                    return None;
                }
                now.with_timezone(&FixedOffset::east_opt(seconds as i32)?)
            }
        };

        Datetime::from_ymd(
            now.year(),
            now.month().try_into().ok()?,
            now.day().try_into().ok()?,
        )
    }
}

fn clean_path(path: &str) -> String {
    path.replace("\\", "/").replace("//", "/").trim_matches('/').to_string()
}

fn get_key(id: FileId) -> String {
    let vpath = id.vpath().get_without_slash().to_string();
    let key = match id.root() {
        VirtualRoot::Project => vpath,
        VirtualRoot::Package(pkg) => {
            format!("{}/{}/{}/{}", pkg.namespace, pkg.name, pkg.version, vpath)
        }
    };
    clean_path(&key)
}

pub fn format_warnings(world: &VirtualWorld, warnings: &[SourceDiagnostic]) -> Vec<String> {
    warnings
        .iter()
        .map(|warning| {
            format_diagnostics(&world, &[], std::slice::from_ref(warning))
                .unwrap_or_else(|_| warning.message.to_string())
        })
        .collect()
}

pub fn format_diagnostics(
    world: &VirtualWorld,
    errors: &[SourceDiagnostic],
    warnings: &[SourceDiagnostic],
) -> Result<String, codespan_reporting::files::Error> {
    let mut w = termcolor::Buffer::no_color();

    let config = term::Config {
        tab_width: 2,
        ..Default::default()
    };

    for diagnostic in warnings.iter().chain(errors.iter()) {
        let diag = match diagnostic.severity {
            Severity::Error => Diagnostic::error(),
            Severity::Warning => Diagnostic::warning(),
        }
        .with_message(diagnostic.message.clone())
        .with_notes(
            diagnostic
                .hints
                .iter()
                .map(|e| (eco_format!("hint: {}", e.v)).into())
                .collect(),
        )
        .with_labels(label(world, diagnostic.span).into_iter().collect());

        term::emit_to_write_style(&mut w, &config, world, &diag)?;

        // Stacktrace-like helper diagnostics.
        for point in &diagnostic.trace {
            let message = point.v.to_string();
            let help = Diagnostic::help()
                .with_message(message)
                .with_labels(label(world, point.span).into_iter().collect());

            term::emit_to_write_style(&mut w, &config, world, &help)?;
        }
    }

    let s = String::from_utf8(w.into_inner()).unwrap();
    Ok(s)
}

/// Create a label for a span.
fn label(world: &VirtualWorld, span: impl Into<DiagSpan>) -> Option<Label<FileId>> {
    let span = span.into();
    Some(Label::primary(span.id()?, world.range(span)?))
}

impl<'a> codespan_reporting::files::Files<'a> for VirtualWorld {
    type FileId = FileId;
    type Name = String;
    type Source = Lines<String>;

    fn name(&'a self, id: FileId) -> CodespanResult<Self::Name> {
        let vpath = id.vpath();
        Ok(if let typst::syntax::VirtualRoot::Package(package) = id.root() {
            format!("{package}{}", vpath.get_with_slash())
        } else {
            vpath.get_without_slash().into()
        })
    }

    fn source(&'a self, id: FileId) -> CodespanResult<Self::Source> {
        Ok(self.lookup(id))
    }

    fn line_index(&'a self, id: FileId, given: usize) -> CodespanResult<usize> {
        let source = self.lookup(id);
        source
            .byte_to_line(given)
            .ok_or_else(|| CodespanError::IndexTooLarge {
                given,
                max: source.len_bytes(),
            })
    }

    fn line_range(&'a self, id: FileId, given: usize) -> CodespanResult<std::ops::Range<usize>> {
        let source = self.lookup(id);
        source
            .line_to_range(given)
            .ok_or_else(|| CodespanError::LineTooLarge {
                given,
                max: source.len_lines(),
            })
    }

    fn column_number(&'a self, id: FileId, _: usize, given: usize) -> CodespanResult<usize> {
        let source = self.lookup(id);
        source.byte_to_column(given).ok_or_else(|| {
            let max = source.len_bytes();
            if given <= max {
                CodespanError::InvalidCharBoundary { given }
            } else {
                CodespanError::IndexTooLarge { given, max }
            }
        })
    }
}

/// Cache discovered fonts.
///
/// A full system font scan can be much longer than compilation itself.
/// `FontStore` cannot be cloned, so the discovered fonts are kept
/// alongside the assembled stores.
struct FontCache {
    /// The locations found by walking the system font directories.
    system: Option<Arc<Vec<(PathBuf, u32, FontInfo)>>>,
    /// The fonts shipped with typst, already parsed.
    embedded: Option<Arc<Vec<(Font, FontInfo)>>>,
}

const EMPTY_FONT_CACHE: FontCache =
    FontCache { system: None, embedded: None };

static FONT_CACHE: Mutex<FontCache> = Mutex::new(EMPTY_FONT_CACHE);

/// Forgets everything discovered so far, so that the next compile picks up
/// fonts installed or removed since.
pub fn clear_font_cache() {
    *FONT_CACHE.lock().unwrap() = EMPTY_FONT_CACHE;
}

/// Reads a cache field, filling it on a miss.
///
/// The lock is not held while building, so two threads missing at once both
/// build and one result is dropped. That wastes a scan but keeps the
/// build time off the lock.
fn cached<T>(
    select: impl Fn(&mut FontCache) -> &mut Option<Arc<T>>,
    build: impl FnOnce() -> T,
) -> Arc<T> {
    let hit = select(&mut FONT_CACHE.lock().unwrap()).clone();
    if let Some(value) = hit {
        return value;
    }

    let value = Arc::new(build());
    *select(&mut FONT_CACHE.lock().unwrap()) = Some(value.clone());
    value
}

pub fn add_fonts(fonts: Vec<Vec<u8>>) {
    for data in fonts.iter() {
        let bytes = Bytes::new(data.to_vec());
        if let Some(font) = Font::iter(bytes).next() {
            LOCAL_FONT_CACHE.lock().unwrap().push(font);
        }
    }
}

pub fn add_font(data: Vec<u8>) {
    let bytes = Bytes::new(data.to_vec());
    if let Some(font) = Font::iter(bytes).next() {
        LOCAL_FONT_CACHE.lock().unwrap().push(font);
    }
}

pub fn prime_font_cache(
    system_fonts: Option<bool>,
    embedded_fonts: Option<bool>,
) {
    let mut font_store = FontStore::new();
    if system_fonts.unwrap_or(true) {
        let system = cached(
            |cache| &mut cache.system,
            || fonts::system().map(|(path, info)| (path.path, path.index, info)).collect(),
        );
        font_store.extend(system.iter().map(|(path, index, info)| {
            (FontPath { path: path.clone(), index: *index }, info.clone())
        }));
    }
    if embedded_fonts.unwrap_or(true) {
        let embedded = cached(|cache| &mut cache.embedded, || fonts::embedded().collect());
        font_store.extend(embedded.iter().cloned());
    }
}

fn system_packages(
    package_path: Option<PathBuf>,
    package_cache_path: Option<PathBuf>,
) -> SystemPackages {
    SystemPackages::from_parts(
        package_path
            .map(FsPackages::new)
            .or_else(FsPackages::system_data),
        package_cache_path
            .map(FsPackages::new)
            .or_else(FsPackages::system_cache),
        UniversePackages::new(crate::download::downloader()),
    )
}

pub fn clear_file_cache() {
    *FILE_CACHE.lock().unwrap() = HashMap::new();
}

pub fn add_file(path: String, data: Vec<u8>) {
    let mut files = FILE_CACHE.lock().unwrap();
    let clean_path = clean_path(path.as_str());
    files.insert(clean_path, Arc::new(FileCache {
        bytes: Bytes::new(data.to_vec()),
        source: Mutex::new(None),
    }));
}

// pub fn add_package_file(namespace: &str, name: &str, version: &str, rel_path: &str, data: &[u8]) {
//     let vfs_rel = clean_path(rel_path);

//     // Use Typst's PackageVersion type to validate the version string
//     let ver: PackageVersion = version.parse().unwrap();

//     // Construct a PackageSpec to ensure consistency with internal Typst logic
//     let spec = PackageSpec {
//         namespace: namespace.into(),
//         name: name.into(),
//         version: ver,
//     };

//     // Matches the format expected by get_vfs_key: "namespace/name/version/path"
//     let full_vfs_key = format!("{}/{}/{}/{}", spec.namespace, spec.name, spec.version, vfs_rel);
//     add_file(full_vfs_key, data.to_vec())
// }
