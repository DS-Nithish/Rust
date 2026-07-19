def recur_fact(n):
    if n==1:
        return 1
    else:
        return n * recur_fact(n-1)
    

num = int(input("Enter a number :"))

if num < 0:
        print("NO NEGATIVE NUMBERS ARE ALLOWED")
    
elif num == 0:
        print("The factorial of 0 is 1")
    
else:
        print("The factorial of",num ,"is",recur_fact(num))