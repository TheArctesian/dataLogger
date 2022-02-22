# Planning 

**Inputs:**
Activity
-a : String = activity 
-t : String(HR.MN) = time 
-sf : String = software
-l : String = location

Emotions
-p : int(x<12) = productivity
-s : int(x<12) = satisfaction
-h : int(x<12) = happiness
-e : int(X<12) = energy
-i : int(x<12) = interest 

Comments
-c : String = comments

**db** 
Sort DB probs by time
Activity Name | Time Spent | Time(when to when | Location | Software |
Productivity Level | Satisfaction Level | Happiness Level | Energy | Interest  


**Sample Commands** 
```Shell
pdb -A "activity" -T "02.00" 
```
or 
promnt the user with Inputs ie 

```Shell
pdb
Activity(string): 
i
Time(HR.MN): 
i 
Location(string): 
i 
Software (string): 
i
Productivity Lebel (int):
i
```

