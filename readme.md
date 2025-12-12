# This program shows a basic CPI to Pump Swap

It should be seen as an instruction on how to use cpi's in your programs logic

If you have the idl for any on-chain program you can use this template to make a CPI 

It is important to note that you don't need to pass in each account with the correct deserialization.
You can also simply use UncheckedAccount on each given account, however this limits your program 
logic, since you can't access the data inside the accounts.


# Why use declare_program!() 

I tried making this work with other methods ( like anchor-gen) and always failed 
This seems to be the best way to interact with a program
*declare_program!() works on-chain AND off-chain* 
