
public class Cheese implements Comparable<Cheese>
{
    private String name;
    private int weight;
    private String colour;
    private boolean smelly;

    public Cheese(String n, int w, String c, boolean s){
        name = n;
        weight = w;
        colour = c;
        smelly = s;
    }

    public String toString(){
        String smellyString;
        if smelly { smellyString = " smelly."; }
        else { smellyString = " not smelly."; }
        return "This " + colour + " cheese weighs " + weight + " grams and is " + smellyString;
    }

    public String getName(){
        return name;
    }

    public int getWeight(){
        return weight;
    }

    public String getColour(){
        return colour;
    }
    
    public boolean isSmelly(){
        return smelly;
    }

    public int compareTo(Cheese other){
        if(!colour.equals(other.colour)){
            return colour.compareTo(other.colour);
        }
        
        return other.weight - weight;
    }
    
}
