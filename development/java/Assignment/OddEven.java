import java.util.*;

public class OddEven{
  public static void main(String[] args) {
   Scanner sc  = new Scanner(System.in);
   System.out.println("Enter a Number");
   int n = sc.nextInt();
     int oddcount = 0;
     int evencount = 0;

   while (n > 0) {
     int odd = n % 10;
     
     if (odd % 2 == 1){
        oddcount++;
     }
     else{
       evencount ++;
     }
    n = n /10;
   }
System.out.println("Odd count: "+oddcount);
System.out.println("Even count: "+evencount);

  }
}
