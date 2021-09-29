import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.scene.control.ScrollPane;
import javafx.scene.layout.BorderPane;
import javafx.scene.layout.GridPane;
import javafx.scene.layout.Pane;
import javafx.scene.layout.StackPane;
import javafx.stage.Stage;
import org.fxmisc.flowless.VirtualizedScrollPane;
import org.fxmisc.richtext.CodeArea;
import org.fxmisc.richtext.LineNumberFactory;


/**
 * Author: Emilia Rose
 * Desc: Starts the GUI
 */

public class Main extends Application {

    @Override
    public void start(Stage primaryStage) throws Exception
    {
        new ViewManager(primaryStage);
    }

    public static void main(String[] args) {
        launch(args);
    }
}
