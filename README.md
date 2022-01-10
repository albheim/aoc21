# Advent of Code 2021
My solutions in Rust for [Advent of Code 2021](https://adventofcode.com/2021).

## Run
To run one of the example you should first have [cargo](https://doc.rust-lang.org/cargo/) installed.
```
cd dayXY
cargo run [a|b]
```

## Timing
One of my goals this year was to make it fast, and inspired by [this](https://www.forrestthewoods.com/blog/solving-advent-of-code-in-under-a-second/) blogpost I aimed for the sum of all runtimes to be under a second.

To time the execution run `./time.sh` to both time individual problems execution as well as a total time. The individual exectuion is plotted below, it is ran on a decent laptop. The total ended up around 0.8 seconds, so the goal was achieved.

```
       ┌                                        ┐ 
    1a ┤■■ 0.009914806                            
    1b ┤■ 0.003422876                             
    2a ┤■ 0.004445009                             
    2b ┤■ 0.003605722                             
    3a ┤■ 0.004392922                             
    3b ┤■ 0.003685164                             
    4a ┤■ 0.005069911                             
    4b ┤■ 0.004737572                             
    5a ┤■■■■ 0.016983322                          
    5b ┤■■■■■■ 0.026020151                        
    6a ┤■ 0.003329775                             
    6b ┤■ 0.003215548                             
    7a ┤■ 0.003641512                             
    7b ┤■ 0.0047716                               
    8a ┤■ 0.003788817                             
    8b ┤■ 0.003611232                             
    9a ┤■ 0.004026641                             
    9b ┤■ 0.003896751                             
   10a ┤■ 0.004496001                             
   10b ┤■ 0.00379699                              
   11a ┤■ 0.004708931                             
   11b ┤■ 0.004346168                             
   12a ┤■■ 0.00818268                             
   12b ┤■■■■■■■■■■■■■■■■■■■■■■■ 0.097723102       
   13a ┤■ 0.005691794                             
   13b ┤■ 0.003450243                             
   14a ┤■ 0.003621869                             
   14b ┤■ 0.004119184                             
   15a ┤■ 0.004735268                             
   15b ┤■■■■■■■■■■■■■■ 0.058125286                
   16a ┤■ 0.003295976                             
   16b ┤■ 0.003292885                             
   17a ┤■ 0.005304372                             
   17b ┤■ 0.004354758                             
   18a ┤■■ 0.006618104                            
   18b ┤■■■■■■■■■ 0.040776272                     
   19a ┤■■■■■ 0.020964782                         
   19b ┤■■■■■ 0.02043519                          
   20a ┤■ 0.00352006                              
   20b ┤■■■■■ 0.022447839                         
   21a ┤■ 0.002796065                             
   21b ┤■■■ 0.014369152                           
   22a ┤■ 0.003268902                             
   22b ┤■■■■■ 0.022190479                         
   23a ┤■■■■■■■■■■■■■■■■■■■■■■■ 0.096960883       
   23b ┤■■■■■■■■■■■■■■■■■■■■■■■■■■■ 0.115971397   
   24a ┤■ 0.003177602                             
   24b ┤■ 0.003488806                             
   25a ┤■■■■■■■■■■■■■■■■■■■■■■■■■■■ 0.114553183   
   25b ┤■ 0.002419652                             
       └                                        ┘ 
```

From my initial implementations there were a few modifications needed to go sub second. The biggest improvement came from day 19, where my first implementation was not only rather simplistic but also did some slightly stupid things. Removing the bad ideas and adding a fingerprint based on internal distances made the runtime go from around 5 seconds to the current 0.02 seconds.

There are offcourse many more improvements that could be made, but for now I will call it enough since the goal was achieved.