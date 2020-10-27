
public class Cheese implements Comparable<Cheese>
{
    private String name;
    private int weight;
    private String colour;

    public Cheese(String n, int w, String c){
        name = n;
        weight = w;
        colour = c;
    }

    public String toString(){
        return weight+ " gramm " +colour+ " "+name;
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

    public int compareTo(Cheese other){
        if(!colour.equals(other.colour)){
            return colour.compareTo(other.colour);
        }
        
        return other.weight - weight;
    }
    
}
