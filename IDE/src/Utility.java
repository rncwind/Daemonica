import javafx.scene.control.IndexRange;
import javafx.scene.control.ScrollPane;
import javafx.scene.layout.Pane;
import javafx.scene.layout.StackPane;
import org.fxmisc.flowless.VirtualizedScrollPane;
import org.fxmisc.richtext.CodeArea;
import org.fxmisc.richtext.LineNumberFactory;

import java.awt.*;
import java.awt.datatransfer.Clipboard;
import java.awt.datatransfer.DataFlavor;
import java.awt.datatransfer.StringSelection;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.util.Base64;

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
        CA.displaceCaret(start.length());
    }

    public static void paste(CodeArea CA)
    {
        Clipboard cb = Toolkit.getDefaultToolkit().getSystemClipboard();
        //Check to see if its a string
        try
        {
            String toPaste = (String) cb.getData(DataFlavor.stringFlavor);
            int pos = CA.getCaretPosition();
            String codezoneText = CA.getText();
            String start = codezoneText.substring(0, pos);
            String end = codezoneText.substring(pos);
            CA.replaceText(start + toPaste + " " + end);
            CA.displaceCaret(start.length() + toPaste.length());
        }
        catch (Exception e)
        {
            //If we get here its not a string
            System.out.println("Not a string");
            //e.printStackTrace();
        }

    }

    public static String MD5Hash(String s)
    {
        try
        {
            MessageDigest md = MessageDigest.getInstance("MD5");
            md.update(s.getBytes());

            return Base64.getEncoder().encodeToString(md.digest());
        }
        catch (Exception e)
        {
            e.printStackTrace();
            return null;
        }
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
