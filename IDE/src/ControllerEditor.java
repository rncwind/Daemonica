import javafx.application.Platform;
import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.stage.FileChooser;
import org.fxmisc.richtext.CodeArea;

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

    public String hash;

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

    @FXML
    public void copyHighlighted(ActionEvent event)
    {
        Utility.copy(codezone);
    }

    @FXML
    public void cutHighlighted(ActionEvent event)
    {
        Utility.cut(codezone);
    }

    @FXML
    public void paste(ActionEvent event)
    {
        Utility.paste(codezone);
    }

}
