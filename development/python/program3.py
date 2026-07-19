N = int(input("Enter a number:"))
arr = []

for i in range(N):
    num = int(input(f"Enter number {i+1}:"))
    arr.append(num)

odd_count = 0
even_count = 0

for num in arr:

    if num % 2 == 0:
        even_count+=1
    else:
        odd_count+=12

print("Even Numbers count:",even_count)
print("Odd Numbers count:",odd_count)
