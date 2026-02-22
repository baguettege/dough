package io.github.bagu.dough.source;

public record SourceRange(
        int start,
        int end
) {
    public SourceRange {
        if (start < 0) throw new IllegalArgumentException(String.valueOf(start));
        if (end < 0) throw new IllegalArgumentException(String.valueOf(end));
        if (end < start) throw new IllegalArgumentException(end + " < " + start);
    }
}
