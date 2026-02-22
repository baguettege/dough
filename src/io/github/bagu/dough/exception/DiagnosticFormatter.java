package io.github.bagu.dough.exception;

import io.github.bagu.dough.source.LineColumn;
import io.github.bagu.dough.source.LineMap;
import io.github.bagu.dough.source.SourceRange;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

public final class DiagnosticFormatter {
    private final DoughException exception;
    private final String src;

    public DiagnosticFormatter(DoughException exception, String src) {
        this.exception = Objects.requireNonNull(exception);
        this.src = src.replace("\r\n", "\n");
    }

    public String format() {
        StringBuilder sb = new StringBuilder();
        sb.append(getHeader());
        List<Line> lines = getLines();

        int maxLineWidth = Integer.toString(lines.getLast().lineNum()).length();
        int gutterWidth = maxLineWidth + 2;
        String gutter = " ".repeat(gutterWidth);

        sb.append("\n").append(gutter).append("|\n");

        for (Line line : lines) {
            sb.append(" ");

            int lineNum = line.lineNum;
            int width = Integer.toString(lineNum).length();

            if (width == maxLineWidth) {
                sb.append(lineNum);
            }
            else {
                sb.append(" ".repeat(maxLineWidth - width))
                        .append(lineNum);
            }

            sb.append(" | ")
                    .append(line.src)
                    .append("\n")
                    .append(gutter)
                    .append("| ")
                    .append(" ".repeat(line.relStart()))
                    .append("^".repeat(line.carets))
                    .append("\n");
        }

        sb.append(gutter).append("|");

        return sb.toString();
    }

    private String getHeader() {
        StringBuilder sb = new StringBuilder();

        SourceRange range = exception.getRange();
        LineMap map = new LineMap(src);
        LineColumn start = map.resolve(range.start());
        LineColumn end = map.resolve(range.end());

        sb.append(exception.getClass().getName())
                .append(": ")
                .append(exception.getMessage())
                .append("\n")

                .append("   ---> ")
                .append(start.line())
                .append(":")
                .append(start.column())
                .append(" - ")
                .append(end.line())
                .append(":")
                .append(end.column());

        return sb.toString();
    }

    private List<Line> getLines() {
        String[] lines = src.split("\n", -1);
        SourceRange range = exception.getRange();

        List<Line> errLines = new ArrayList<>();
        int lineStart = 0;
        int lineNum = 1;

        for (String line : lines) {
            int lineEnd = lineStart + line.length();

            if (range.start() <= lineEnd && range.end() >= lineStart) {

                int relStart = Math.max(range.start() - lineStart, 0);
                int relEnd = Math.min(range.end() - lineStart, line.length());
                int carets = relEnd - relStart;
                if (carets == 0) carets = 1;

                errLines.add(new Line(line, lineNum, relStart, carets));
            }

            lineStart = lineEnd + 1;
            lineNum++;
        }

        return Collections.unmodifiableList(errLines);
    }

    private record Line(
            String src,
            int lineNum,
            int relStart,
            int carets
    ) {}
}
