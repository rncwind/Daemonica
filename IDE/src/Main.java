import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.scene.control.ScrollPane;
import javafx.scene.layout.BorderPane;
import javafx.scene.layout.GridPane;
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
        new ViewManager();

        //This would normally be cast into Parent class, however BorderPane provides access to a usually protected method
        BorderPane root = FXMLLoader.load(getClass().getResource("repl.fxml"));

        //JavaFX is a poor excuse for a library, who even thinks this is sensible?
        //Modifies the CodeArea container provided by RichTextFX to have line numbers
        //This can only be done outside of the FXML definition
        CodeArea top_code = ViewManager.getCR().inputcode;
        top_code.setParagraphGraphicFactory(LineNumberFactory.get(top_code));
        GridPane GP = (GridPane) ((root.getChildren()).get(0));
        ScrollPane SP = (ScrollPane) ((GP.getChildren()).get(0));
        SP.setContent(new StackPane(new VirtualizedScrollPane<>(top_code)));



        primaryStage.setTitle("Daemonium Bibliotheca");
        primaryStage.setScene(new Scene(root, 300, 275));
        primaryStage.show();
    }

    public static void main(String[] args) {
        launch(args);
    }
}
