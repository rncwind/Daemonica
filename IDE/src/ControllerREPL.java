import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.stage.FileChooser;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.stream.Collectors;

/**
 * Author: Emilia Rose
 * Desc: Controller for FXML GUI
 */

public class ControllerREPL
{

    //  --REPL Controller Items--
    /**
     * Opens the editor scene upon clicking 'new file' in the REPL
     * @param event
     */
    @FXML
    public void openEditor(ActionEvent event)
    {
        ViewManager.editor_view("","New File",null);
    }


    private void loadIntoText(File selectedFile)
    {
        try
        {
            // Buffered Reader is much more efficient than standard file reader due to buffering
            FileReader fr = new FileReader(selectedFile);
            BufferedReader br = new BufferedReader(fr);

            //Now to read line by line
            // This works by gathering all the individual lines and joining while retaining line ends
            String text = br.lines().collect(Collectors.joining(System.lineSeparator()));

            //Not sure we need these for the br since .lines() *should* close it but just in case
            br.close();
            fr.close();

            ViewManager.editor_view(text,"Editing:" + selectedFile.getName(),selectedFile);
        }
        catch (Exception e)
        {
            e.printStackTrace();
        }
    }


    /**
     *
     * @param event
     */
    @FXML
    public void chooseFile(ActionEvent event)
    {
        try
        {
            FileChooser fileSelector = new FileChooser();
            fileSelector.setTitle("Select source file...");
            //only allow .ritual
            fileSelector.getExtensionFilters().addAll
                    (
                            new FileChooser.ExtensionFilter("Ritual", "*.ritual"),
                            new FileChooser.ExtensionFilter("Text", "*.txt")
                    );
            //Allow selection of file and check that
            File selectedFile = fileSelector.showOpenDialog(null);
            if (selectedFile != null && selectedFile.isFile()) {
                //carry on
                loadIntoText(selectedFile);
            }
        }
        catch (Exception e)
        {
            e.printStackTrace();
        }
    }
}
