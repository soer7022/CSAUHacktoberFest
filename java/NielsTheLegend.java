
/**
 * Lav en beskrivelse af klassen NielsTheLegend her.
 * 
 * @author (dit navn her)
 * @version (versions nummer eller dato her)
 */
public class NielsTheLegend
{
    private String name;
    public NielsTheLegend(String name) {
        this.name = name;   
    }

    public String isThisNiels() {
        if(name.equals("Niels")) {
            System.out.println("Nu bliver det rigtig vildt");
        }
        return "You are not awesome as Niels and you suck"; 
    }
    
    public String mathIsGoingCrazy() {
        System.out.println("What is the sqrt of 2?");
        System.out.println("if we try to calculate, math IS GOING CRAZY");
        System.out.println("Alle I have left to say is:");
        System.out.println("https://www.youtube.com/watch?v=oiETVvdDa7Y");
        return null;
    }
}
