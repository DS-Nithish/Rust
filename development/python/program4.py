class StringReverser:
    def reverse_words(self,text):
        words = text.split()
        reversed_words = words[::-1]
        return ''.join(reversed_words)

reverser = StringReverser()
result = reverser.reverse_words("Hello World")
print(result)