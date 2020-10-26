
/**
 * Write a description of class Food here.
 *
 * @author Anne Kirstine Overgaard
 * @version (a version number or a date)
 */
public class Food implements Comparable<Food> {
    
    private String name;
    private int weight;
    private int price;

    public Food(String name, int weight, int price) {
        this.name = name;
        this.price=price;
        this.weight = weight;
    }

    public String toString() {
        return weight + " gram " + name + " for " + price + " DKK";
    }

    public int getWeight() {
        return weight;
    }

    public int getPrice(){
        return price;
    }

    public int compareTo(Food other){
        if(!name.equals(other.name)){
            return name.compareTo(other.name);
        }
        return other.weight - weight;
    }
}
