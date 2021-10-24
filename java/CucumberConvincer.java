
/**
 * If you dont think cucumbers are nice you will be convinced in one minute.
 *
 * @author (Simon Bech)
 * @version (1.0)
 */
public class CucumberConvincer
{
     public void doYouThinkCucumbersAreNice(String n){
        if(n.equals("yes")){
            System.out.println("Good, no need to convince you then");
        } 
        else if(n.equals("no")){
            System.out.println("Then watch this: https://www.youtube.com/watch?v=lL42O2p1nx8");
        }
        else{
            System.out.println("You have to enter yes or no");
        }
    }
}
