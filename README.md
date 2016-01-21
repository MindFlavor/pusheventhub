# Push to Event Hub

[![Build Status](https://travis-ci.org/MindFlavor/pusheventhub.svg?branch=master)](https://travis-ci.org/MindFlavor/pusheventhub) [![stable](http://badges.github.io/stability-badges/dist/stable.svg)](http://github.com/badges/stability-badges)

### Rust command line program to push events in Microsoft Azure Event Hub. 
For further details please refer to this blob post [TODO](TODO).

## Goal
This program is not meant to be used in production, it's a proof-of-concept. Feel free to build upon it.


## Usage

```
pusheventhub.exe [FLAGS] --ns <NAMESPACE> --eh <EVENT_HUB> --pn <POLICY_NAME> --pk <POLICY_KEY> --fn <INPUT_FILE_NAME>
```

### Required parameters
|Flag|Name|Description|
|---|---|---|
| ```ns``` | ```<NAMESPACE>``` |Azure Service bus namespace|
| ```eh``` | ```<EVENT_HUB>``` |Azure event hub name|
| ```pn``` | ```<POLICY_NAME>``` |Azure event hub policy name|
| ```pk``` | ```<POLICY_KEY>``` |Azure event hub policy shared key|
| ```fn``` | ```<INPUT_FILE_NAME>``` |Event body file name (your JSON payload)|

## License
The code is under the Apache License, version 2.0. See the [LICENSE](LICENSE) file for further details.
