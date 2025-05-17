class GetTextProcessor < HexaPDF::Content::Processor
  def initialize
    super
    @text_blocks = []
  end

  def show_text_with_positioning(str)
    text_pos = decode_text_with_positioning(str)

    @text_blocks << {
        x: text_pos.upper_left[0],
        y: text_pos.upper_left[1],
        height: (text_pos.upper_left[1] - text_pos.lower_left[1]).abs,
        index: @text_blocks.length,
        text: text_pos.string,
    }
  end

  def string
    prev_block = nil
    text = []

    def on_same_line?(block1, block2)
      ## consider blocks on same line when vertical distance is less than 0.2 of block height
      (block1[:y] - block2[:y]).abs < [block1[:height], block2[:height]].max * 0.2
    end

    @text_blocks.sort do |left, right|
      if on_same_line?(left, right)
        [left[:x], left[:index]] <=> [right[:x], right[:index]]
      else
        right[:y] - left[:y] # reverse order
      end
    end.each do |block|
      unless prev_block.nil?
        text << (on_same_line?(prev_block, block) ? ' ' : "\n")
      end

      text << block[:text]
      prev_block = block
    end

    text.join + "\n"
  end
end
