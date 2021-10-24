
/**
 * Lav en beskrivelse af klassen Driver her.
 * 
 * @author Anna Hallenberg
 * @version 11-10-2019
 */
public class Driver
{
    public static void test() {
        Food f1 = new Food("ost", 300, 15);
        Food f2 = new Food("laks", 200, 10);
        Food f3 = new Food("ost", 200, 30);
        Food f4 = new Food("reje", 400, 40);
        Food f5 = new Food("kød", 200, 45);
        
        System.out.println(f1);
        System.out.println(f2);
        System.out.println(f3);
        System.out.println(f4);
        System.out.println(f5);
        
        //Test 2
        DeepFreezer df = new DeepFreezer("AU");
        df.addFood(f1);
        df.addFood(f2);
        df.addFood(f3);
        df.addFood(f4);
        df.addFood(f5);
        
        System.out.println("");
        System.out.println("Antallet af mad der vejer under 300 (res = 4)");
        System.out.println(df.lightMeals(300));
        
        //Test 3
        System.out.println("");
        System.out.println("returnere dyreste madvare (res = ost 45kr)");
        System.out.println(df.expensiveFood());
        
        // Test 4
        System.out.println("");
        System.out.println("Print DeepFreezer");
        df.printDeepFreezer();
        
        //Test 5
        System.out.println("");
        System.out.println("Find madvare med min. vægt på 250 gram");
        System.out.println(df.findFood(250));
        
        //Test 6
        System.out.println("");
        System.out.println("Find billigste madvare i interval [200,300]");
        System.out.println(df.cheapFood(200, 300));
    }
}


