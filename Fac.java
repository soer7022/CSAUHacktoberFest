public class Fac {
    
    public static int fac(int i) {
        if(i <0) {
            System.out.println("i must not be below 0");
            return 0;
        } 
        else {
            if(i<=1) {
                return 1;
            } else return i*fac(i-1);
        }
    }

}
