import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.ScrollPane;
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

    ViewManager()
    {
        try
        {
            FXMLLoader loader = new FXMLLoader();
            loader.setLocation(getClass().getResource("repl.fxml"));
            loader.load();
            CR = loader.getController();

            FXMLLoader loaderFE = new FXMLLoader();
            loaderFE.setLocation(getClass().getResource("FileEditor.fxml"));
            BorderPane EP = loaderFE.load();
            CE = loaderFE.getController();

            //Sets the CodeAreas to have line numbers for the Editor
            CodeArea CA = CE.codezone;
            CA.setParagraphGraphicFactory(LineNumberFactory.get(CA));
            ScrollPane SP = (ScrollPane) EP.getChildren().get(0);
            SP.setContent(new StackPane(new VirtualizedScrollPane<>(CE.codezone)));


            EditorStage = createStage(EP, "Daemonium Bibliotheca Editor");
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
        stage.setScene(new Scene(P, 500, 400));
        return stage;
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
