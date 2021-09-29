import javafx.scene.control.IndexRange;
import javafx.scene.control.ScrollPane;
import javafx.scene.layout.Pane;
import javafx.scene.layout.StackPane;
import org.fxmisc.flowless.VirtualizedScrollPane;
import org.fxmisc.richtext.CodeArea;
import org.fxmisc.richtext.LineNumberFactory;

import java.awt.*;
import java.awt.datatransfer.Clipboard;
import java.awt.datatransfer.StringSelection;

public class Utility
{
    public static void copy(CodeArea CA)
    {
        String selected = CA.getSelectedText();
        StringSelection ss = new StringSelection(selected);
        Clipboard cb = Toolkit.getDefaultToolkit().getSystemClipboard();
        cb.setContents(ss, null);
    }

    public static void cut(CodeArea CA)
    {
        copy(CA);
        //Select the text, get the substrings to contruct a string excluding the region cut, then replace codezone text
        IndexRange range = CA.getSelection();
        String codezoneText = CA.getText();
        String start = codezoneText.substring(0, range.getStart());
        String end = codezoneText.substring(range.getEnd());
        CA.replaceText(start + end);
    }

    public static void addLineNums(CodeArea CA, Pane P)
    {
        try
        {
            //Sets the CodeAreas to have line numbers for the Editor
            CA.setParagraphGraphicFactory(LineNumberFactory.get(CA));
            ScrollPane SP = (ScrollPane) P.getChildren().get(0);
            SP.setContent(new StackPane(new VirtualizedScrollPane<>(CA)));
        }
        catch (Exception e)
        {
            e.printStackTrace();
        }
    }
}
