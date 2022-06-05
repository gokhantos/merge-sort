def main():

    import random

    #Open a file named numbersmake.txt.
    outfile = open('numbers.txt', 'a+')

    #Produce the numbers
    for count in range(1000000):
        #Get a random number.
        num = random.randint(1, 10000)
        outfile.write(str(num)+"\n")
    
    #Close the file.
    outfile.close()
    print('Data written to numbers.txt')

#Call the main function
main()