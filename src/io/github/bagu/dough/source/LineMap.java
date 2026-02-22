package io.github.bagu.dough.source;

import java.util.ArrayList;
import java.util.List;

public final class LineMap {
    private final int[] offsets;

    public LineMap(String src) {
        src = src.replace("\r\n", "\n");
        List<Integer> offsets = new ArrayList<>();
        offsets.add(0);

        for (int i = 0; i < src.length(); i++) {
            if (src.charAt(i) == '\n') {
                offsets.add(i + 1);
            }
        }

        this.offsets = offsets.stream().mapToInt(Integer::intValue).toArray();
    }

    public int getLine(int pos) {
        int line = 1;

        for (int offset : offsets) {
            if (pos < offset) break;
            line++;
        }

        return line;
    }

    public int getColumn(int pos) {
        int lineOffset = 0;

        for (int offset : offsets) {
            if (pos < offset) break;
            lineOffset = offset;
        }

        return pos - lineOffset + 1;
    }

    public LineColumn resolve(int pos) {
        return new LineColumn(getLine(pos), getColumn(pos));
    }
}
