# ct

A filter command that postfixes lines by the number of occurrences. 
Differently from `sort | uniq -c`, you don't need to sort input lines. 

Currently, this command is just the following gawk script. 

```js
{a[$0]++}END{for(k in a)print k,a[k]}
```


```bash
$ echo a b c a b c a | xargs -n 1 | awk '{a[$0]++}END{for(k in a)print k,a[k]}'
a 3
b 2
c 2
$ echo a b c a b c a | xargs -n 1 | ct
a 3
b 2
c 2
```

## Next

This command will be implemented with Rust. 


## License 

This command is distributed under GPL-3.0 or later. But it will be done under
GPL or MIT dual license. 

