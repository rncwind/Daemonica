import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.fxml.FXMLLoader;
import javafx.scene.control.ScrollPane;
import javafx.scene.control.TabPane;
import javafx.scene.control.TextArea;
import javafx.scene.layout.AnchorPane;
import javafx.scene.layout.BorderPane;
import javafx.scene.layout.StackPane;
import javafx.stage.FileChooser;
import org.fxmisc.flowless.VirtualizedScrollPane;
import org.fxmisc.richtext.CodeArea;
import org.fxmisc.richtext.LineNumberFactory;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;

/**
 * Author: Emilia Rose
 */

public class ControllerEditor
{
    @FXML
    public CodeArea codezone;

    public File currentFile;

    /**
     * Saves a file being edited to disk
     * @param location absolute path where the file will be saved
     */
    private void saveToDisk(File location)
    {
        if (location != null)
        {
            try
            {
                BufferedWriter writer = new BufferedWriter(new FileWriter(location));
                writer.write(codezone.getText());
                writer.close();
                currentFile = location;
                ViewManager.getEditorStage().setTitle("Editing: " + currentFile.getName());
            }
            catch (Exception e)
            {
                e.printStackTrace();
            }
        }
    }

    @FXML //  Will overwrite if possible, otherwise functions the same as save as
    public void saveFile(ActionEvent event)
    {
        if (currentFile != null)
        {
            saveToDisk(currentFile);
        }
        else
        {
            System.out.println("NO PATH");
            saveAsFile(null);
        }
    }

    @FXML // Saves to disk without overwriting
    public void saveAsFile(ActionEvent event)
    {
        FileChooser fileSelector = new FileChooser();
        fileSelector.setTitle("Save As");
        fileSelector.getExtensionFilters().addAll(new FileChooser.ExtensionFilter("Ritual", "*.ritual"));
        File selectedPath = fileSelector.showSaveDialog(null);
        saveToDisk(selectedPath);
    }

    public void addLineNums(CodeArea CA)
    {
        try
        {
            //Sets the CodeAreas to have line numbers for the Editor
            BorderPane EP = (BorderPane) ViewManager.getEditorStage().getScene().getRoot();
            CA.setParagraphGraphicFactory(LineNumberFactory.get(CA));
            ScrollPane SP = (ScrollPane) EP.getChildren().get(0);
            SP.setContent(new StackPane(new VirtualizedScrollPane<>(CA)));
        }
        catch (Exception e)
        {
            e.printStackTrace();
        }
    }
}
