import java.util.*;

public class DigitManipulation{
  public static void main(String[] args){
    Scanner sc = new Scanner(System.in);
    int number = 234 ; 
    System.out.println(ProductofDigits(number));
  }
  static int  ProductofDigits(int n){
    int 

    while(n>0){
      int ld = n % 10;
      n*=n;
      n = ld/10;
    return n;

    }

  }
}
