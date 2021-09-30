import javafx.event.ActionEvent;
import javafx.fxml.FXML;

public class ControllerSavePopup
{
    @FXML
    public void save(ActionEvent event)
    {
        ViewManager.getCE().saveFile(null);
        exit(null);
    }

    @FXML
    public void saveAs(ActionEvent event)
    {
        ViewManager.getCE().saveAsFile(null);
        exit(null);
    }

    @FXML
    public void exit(ActionEvent event)
    {
        ViewManager.getSaveStage().close();
    }


}
