import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.Label;
import javafx.scene.control.ScrollPane;
import javafx.scene.control.Tab;
import javafx.scene.control.TabPane;
import javafx.scene.layout.*;
import javafx.stage.Stage;
import org.fxmisc.flowless.VirtualizedScrollPane;
import org.fxmisc.richtext.CodeArea;
import org.fxmisc.richtext.LineNumberFactory;

import java.io.File;

/**
 * Author: Emilia Rose
 * Desc: Manages transition between views, and pointers to controllers
 */

public class ViewManager
{
    private static ControllerREPL CR;
    private static ControllerEditor CE;

    private static Stage EditorStage;



    ViewManager(Stage ps)
    {
        try
        {
            FXMLLoader loader = new FXMLLoader();
            loader.setLocation(getClass().getResource("repl.fxml"));
            BorderPane P = loader.load();
            CR = loader.getController();

            createStage(ps, P,"Daemonium Bibliotheca");
            ps.show();


            FXMLLoader loaderFE = new FXMLLoader();
            loaderFE.setLocation(getClass().getResource("FileEditor.fxml"));
            BorderPane EP = loaderFE.load();
            CE = loaderFE.getController();

            EditorStage = createStage(EP, "Daemonium Bibliotheca Editor");

            Utility.addLineNums(CE.codezone, (Pane) EP);
            Utility.addLineNums(CR.inputcode, (Pane) (P.getChildren().get(0)));
        }
        catch (Exception e)
        {
            e.printStackTrace();
        }
    }

    public static ControllerEditor getCE()
    {
        return CE;
    }

    public static ControllerREPL getCR()
    {
        return CR;
    }

    public static Stage getEditorStage()
    {
        return EditorStage;
    }

    private Stage createStage(Parent P, String title)
    {
        //Create Scene for Editor
        Stage stage = new Stage();
        stage.setTitle(title);
        stage.setScene(new Scene(P, 400, 600));
        return stage;
    }

    private void createStage(Stage s, Parent P, String title)
    {
        //Create Scene for Editor
        s.setTitle(title);
        s.setScene(new Scene(P, 400, 600));
    }

    public static void editor_view(String text, String title, File currentFile)
    {
        //Allows the stage to be kept
        CE.codezone.replaceText(text);
        CE.currentFile = currentFile;
        EditorStage.setTitle(title);
        EditorStage.show();
    }
}
