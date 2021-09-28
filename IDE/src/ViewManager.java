import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.stage.Stage;

public class ViewManager
{
    public static ControllerREPL CR;
    public static ControllerEditor CE;

    public static Stage EditorStage;

    ViewManager()
    {
        System.out.println("Constructing");
        try
        {
            FXMLLoader loader = new FXMLLoader();
            loader.setLocation(getClass().getResource("repl.fxml"));
            loader.load();
            CR = loader.getController();


            System.out.println("hi");
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

    private Stage createStage(Parent P, String title)
    {
        //Create Scene for Editor
        Stage stage = new Stage();
        stage.setTitle(title);
        stage.setScene(new Scene(P, 500, 400));
        return stage;
    }

    public static void editor_view()
    {
        //Allows the stage to be kept
        CE.codezone.setText("");
        EditorStage.show();
    }
}
