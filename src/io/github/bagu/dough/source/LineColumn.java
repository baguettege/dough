package io.github.bagu.dough.source;

public record LineColumn(
        int line,
        int column
) {
    public LineColumn {
        if (line < 0) throw new IllegalArgumentException(String.valueOf(line));
        if (column < 0) throw new IllegalArgumentException(String.valueOf(column));
    }
}
