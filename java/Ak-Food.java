
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
    private boolean vegetarian;

    public Food(String name, int weight, int price, boolean vegetarian) {
        this.name = name;
        this.price = price;
        this.weight = weight;
        this.vegetarian = vegetarian;
    }

    public String toString() {
        String vegString = "";
        if isVegetarian() { vegString = "vegetarian "; }
        return weight + " g of " + vegString + name + " for " + price + " DKK";
    }

    public int getWeight() {
        return weight;
    }

    public int getPrice() {
        return price;
    }
    
    public boolean isVegetarian() {
        return vegetarian;
    }

    public int compareTo(Food other){
        if(!name.equals(other.name)){
            return name.compareTo(other.name);
        }
        return other.weight - weight;
    }
}
