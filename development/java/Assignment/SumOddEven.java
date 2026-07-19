import java.util.*;

public class OddEven{
  public static void main(String[] args) {
   Scanner sc  = new Scanner(System.in);
   System.out.println("Enter a Number");
   int n = sc.nextInt();
     int oddsum = 0;
     int evensum = 0;

   while (n > 0) {
     int odd = n % 10;
     
     if (odd % 2 == 0){
        evensum = odd + evensum;
     }
     else if(odd % 2 == 1){
       oddsum = oddsum+ odd;
      
     }
     else{
     System.out.println("noo");}
    n = n /10;
   }
   System.out.println("Even Sum: "+evensum);
   System.out.println("Odd Sum: "+oddsum);

  }
}
