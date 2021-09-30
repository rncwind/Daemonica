import javafx.application.Application;
import javafx.stage.Stage;



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
