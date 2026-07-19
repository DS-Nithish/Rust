import java.util.*;

public class EvenSum{
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    int n,sum=0,product = 1;
    n = 1200;

    System.out.println(n);
    while(n>0){
      int ld = n%10;
      sum += ld;
      product*=ld;
      System.out.println(ld);
      n = n/10;
    }
    System.out.println("Sum:"+sum);
    System.out.println("Product:"+product);

  }
}
