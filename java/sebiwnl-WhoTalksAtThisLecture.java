
/**
 * Lav en beskrivelse af klassen WhoTalksAtThisLecture her.
 * 
 * @author (dit navn her)
 * @version (versions nummer eller dato her)
 */
public class WhoTalksAtThisLecture
{
    public String IsThereDSAUAndLEGOOnTheFBPoster(String yesORno) {
        if(yesORno.equals("y")) {
            return "Its 100 % Søren";
        } 
        return "Its 100 % NOT Søren";
    }
    
    public String doesItHaveSomethingToDoWithDSAU(String yesORno) {
        if(yesORno.equals("y")) {
            return "50 % its Søren";
        } 
        return "50 % its NOT Søren";
    }
    
    public String doesItHaveSomethingToDoWithLEGO(String yesORno) {
        if(yesORno.equals("y")) {
            return "50 % its Søren";
        } 
        return "50 % its NOT Søren";
    }
}
