import java.util.*;

public class NeonNumber {
   public static void main(String[] args) {
      Scanner sc = new Scanner(System.in);
      System.out.println("Enter a number");
      int n = sc.nextInt();
      
      int square = n*n;//9
      int sum = 0;

      while(square>0){
        int ld = square%10;//1,8
        sum = sum + ld;//9
        square = square /10;
      }
        if(sum == n){
          System.out.println("Neon Number");

        }
        else{
          System.out.println("Not a Neon Number");
        }

  } 
}
