import java.util.*;
import java.util.List;
import java.util.Collections;
import java.util.stream.Collectors;

/**
 * Write a description of class Airport here.
 *
 * @author (your name)
 * @version (a version number or a date)
 */
public class Airport
{
    private String name;
    private ArrayList<Flight> flights;

    public Airport(String name){
        this.name = name;

        flights = new ArrayList<Flight>();
    }

    public void addFlight(Flight f){
        flights.add(f);
    }

    public int findSeats(String airline){
        int totalSeats = 0;
        for(Flight a : flights){
            if(a.getAirline().equals(airline)){
                totalSeats = a.getSeats() + totalSeats;
            }
        }
        return totalSeats;
    }

    public Flight fastFlightFrom(String airline){
        Flight fastest = new Flight("", 0, 0);

        for(Flight a : flights){
            if(a.getAirline().equals(airline) && a.getSpeed() > fastest.getSpeed() 
            || fastest.getAirline().equals("")){
                fastest = a;
            }
        }
        return fastest;
    }

    public void printAirport(){
        System.out.println(name);

        Collections.sort(flights);

        for(Flight f : flights){
            System.out.println(f);
        }
    }
    
    public List<Flight> findFlights(int min, int max){
      return flights
      .stream()
      .filter(f -> f.getSpeed() >= min & f.getSpeed() <= max)
      .collect(Collectors.toList());
    }
    
    public long fastFlights(int minSpeed){
        long fastFlights = flights.stream()
        .filter(f -> f.getSpeed() >= minSpeed)
        .count();
        return fastFlights;
    }
}
