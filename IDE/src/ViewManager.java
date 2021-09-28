import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.stage.Stage;

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
            Parent EP = loaderFE.load();
            CE = loaderFE.getController();

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
        CE.codezone.setText(text);
        CE.currentFile = currentFile;
        EditorStage.setTitle(title);
        EditorStage.show();
    }
}
