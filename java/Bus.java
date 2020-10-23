

/**
 * Lav en beskrivelse af klassen Bus her.
 * 
 * @author Pedro
 * @version (versions nummer eller dato her)
 */
public class Bus implements Comparable <Bus>
{
    private String departure;
    private String destination;
    private int seats;
   
    
    public Bus(String departure,String destination, int  seats){
        this.departure = departure;
        this.destination = destination;
        this.seats = seats;
    }
    
    public String toString(){
        return departure + " --> " + destination
                + " with " + seats + " seats";
    }
    
    public String getDeparture(){
        return departure;
    }
    
    public String getDestination(){
        return destination;
    }
    
    public int getSeats(){
        return seats;
    }
    
    public int compareTo(Bus other){
        if(destination.equals(other.destination)){
            if(seats != other.seats){
                return other.seats - seats;
            }
        }
        return destination.compareTo(other.destination);
    }
}
