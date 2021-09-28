import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.ScrollPane;
import javafx.scene.layout.BorderPane;
import javafx.scene.layout.GridPane;
import javafx.scene.layout.StackPane;
import javafx.scene.layout.VBox;
import javafx.stage.Stage;
import org.fxmisc.flowless.VirtualizedScrollPane;
import org.fxmisc.richtext.CodeArea;
import org.fxmisc.richtext.LineNumberFactory;
import org.fxmisc.richtext.StyleClassedTextArea;

import javax.swing.text.View;

/**
 * Author: Emilia Rose
 * Desc: Starts the GUI
 */

public class Main extends Application {

    @Override
    public void start(Stage primaryStage) throws Exception
    {
        new ViewManager();


        BorderPane root = FXMLLoader.load(getClass().getResource("repl.fxml"));

        //JavaFX is a poor excuse for a library, who even thinks this is sensible?
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
