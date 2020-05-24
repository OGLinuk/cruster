# Code Review ~ 0

## Reviewer: [Pigeonhands](https://github.com/Pigeonhands)

## Reviewee: [OGLinuk](https://github.com/OGLinuk)

### Problem

The ***problem*** encountered when the number of urls to crawl increases exponentially, the length of execution for the program increased exponentially. For example any 2 base urls could yield 100+ links *each*. Those 200 could produce 10000+ each, and so on. The current state of cruster loops through each base url found in config.toml and calls the crawl method. This means that the more urls the longer it will take and bottleneck when only crawling one url at a time. 

### Proposed solution

To solve this problem Bahnahnah suggested implementing the [threadpooling](https://en.wikipedia.org/wiki/Thread_pool) design pattern. What this enables is the crawls to execute concurrently. This means that rather than one crawl running after another, multiple crawls run at the same or similar times. 

## Implementation notes

---

## [main.rs](https://github.com/OGLinuk/cruster/commit/8d069d37573a91e7132726b62cbf8cd45668ae7d#diff-639fbc4ef05b315af92b4d836c31b023)

#### try_main()

```Rust
for r in rx.iter().take(n_jobs) {
    for x in r {
        raw_url_writer.write(&x);
    }
}
```

refactored to

```Rust
for r in rx.iter().take(n_jobs).flatten() {
    raw_url_writer.write(&r);
}
```

which I refactored to

```Rust
rec_chan.iter()
    .take(n_jobs)
    .flatten()
    .for_each(|x| raw_url_writer.write(&x));
```

##### Notes

Apply [```.flatten()```](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten) to the iterator, which flattens nested structure, removing the need for a second for loop.

```Rust
let str_url = url.as_str();
if str_url.contains("%") {
    let decoded_url = decode(&str_url);
    writeln!(url_file.file, "{}", decoded_url.unwrap())
                            .expect("could not write");
} else {
    writeln!(url_file.file, "{}", str_url)
                            .expect("could not write str_url");
}
```

refactored to

```Rust
let decoded_url = decode(url.as_str())
                            .expect("failed to decode");
writeln!(url_file.file, "{}", decoded_url)
                            .expect("could not write");
```

##### Notes

Decode the url regardless, no point in checking for %.

---

## [urlwriter.rs](https://github.com/OGLinuk/cruster/commit/8d069d37573a91e7132726b62cbf8cd45668ae7d#diff-7dfe6878c2535abd225f41aaa8fdc7e1)


#### UrlWriter.aggregate_roots()

```Rust
for line in BufReader::new(file).lines() {
    hset.insert(line?);
}
hset.into_iter().for_each(|v| vhset.push(v.to_owned()));
```

refactored to

```Rust
let unique_lines = BufReader::new(file)
    .lines()
    .map(|l| l.unwrap())
    .filter(|l| uniques.insert(l.to_owned()));

base_urls.extend(unique_lines);
```

##### Notes

Utilizing [```.lines()```](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines)[```.map()```](https://doc.rust-lang.org/std/iter/struct.Map.html)[```.filter()```](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) to replace the need for 2 loops. The second loop is replaced by the ```.map()``` method which maps ```l``` to the [```.unwrap()```](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap) returned value, then ```.filter()``` the returned value of ```.insert()``` using ```l```. If the value is unique (returned true) the value will added be to the unique_lines value. 

---

## [crawler.rs](https://github.com/OGLinuk/cruster/commit/8d069d37573a91e7132726b62cbf8cd45668ae7d#diff-28330c24e8d3f654df8e3c5a6df2e4b0)


added
```Rust
pub fn from_url_string(str_url: &str) -> Result<Crawler> {
    Ok(Crawler::new(Url::parse(str_url)?))
}
```

resulted (in [main.rs](https://github.com/OGLinuk/cruster/commit/8d069d37573a91e7132726b62cbf8cd45668ae7d#diff-639fbc4ef05b315af92b4d836c31b023))

```Rust
let to_crawl = Url::parse(url)?;
let c = Crawler::new(to_crawl);
```

refactored to 

```Rust
let c = Crawler::from_url_string(&url)?;
```

##### Notes

Added ```from_url_string()``` to eliminate the need to parse the value of url before passing to ```Crawler::new()```.

---

## [config.rs](https://github.com/OGLinuk/cruster/blob/master/src/utils/config.rs)

```Rust
pub fn load(f: &Path) -> Result<Config> { 
    // load file
}

pub fn save(&self, p: &Path) -> Result<()> {
    // save file
}
```

Refactored to 

```Rust
pub fn load<T: AsRef<Path>>(f: T) -> Result<Config> {
    // load file
}

pub fn save<T: AsRef<Path>>(&self, p: T) -> Result<()> {
    // save file
}
```

resulted (resulted in [config.rs]())

```Rust
let c = Config::new(vhset, 4);
c.save(Path::new("config.toml"))?;
```

refactored to

```Rust
Config::new(base_urls, 4).save("config.toml")?;
```

##### Notes

Refactored the type required for the ```.load()``` and ```.save()``` parameters to [```AsRef```](https://doc.rust-lang.org/std/convert/trait.AsRef.html), which eliminated the need to parse before passing a value.
