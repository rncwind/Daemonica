import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.layout.*;
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
    private static ControllerSavePopup CSP;

    private static Stage EditorStage;
    private static Stage SaveStage;



    ViewManager(Stage ps)
    {
        try
        {
            //REPL
            FXMLLoader loader = new FXMLLoader();
            loader.setLocation(getClass().getResource("repl.fxml"));
            BorderPane P = loader.load();
            CR = loader.getController();
            createStage(ps, P,"Daemonium Bibliotheca");
            ps.show();

            //Editor
            FXMLLoader loaderFE = new FXMLLoader();
            loaderFE.setLocation(getClass().getResource("FileEditor.fxml"));
            BorderPane EP = loaderFE.load();
            CE = loaderFE.getController();
            EditorStage = createStage(EP, "Daemonium Bibliotheca Editor", 400, 600);

            //Save popup
            FXMLLoader loaderS = new FXMLLoader();
            loaderS.setLocation(getClass().getResource("SavePopup.fxml"));
            Pane SP = loaderS.load();
            CSP = loaderS.getController();
            SaveStage = createStage(SP, "Do you want to save?", 300,70);


            //Lambda to do sneaky code whenever stage closed
            EditorStage.setOnHiding( event ->
            {
                String comp = Utility.MD5Hash(CE.codezone.getText());
                if (!comp.equals(CE.hash))
                {
                    System.out.println("File changed");
                    SaveStage.showAndWait();
                }
            } );


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

    public static Stage getSaveStage()
    {
        return SaveStage;
    }

    private Stage createStage(Parent P, String title, int hor, int ver)
    {
        //Create Scene for Editor
        Stage stage = new Stage();
        stage.setTitle(title);
        stage.setScene(new Scene(P, hor, ver));
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

        //Due to how .getText() formats the return, it has to be used for the hash otherwise it will mismatch even
        // without any text changes
        CE.hash = Utility.MD5Hash(CE.codezone.getText());
        EditorStage.setTitle(title);
        EditorStage.show();
    }
}
