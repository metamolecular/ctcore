# CTcore

CTcore is a suite of precision tools for reading and ultimately writing the CTfile family of formats found throughout computational chemistry and cheminformatics. For details, see [Reading CTfiles with CTcore](https://depth-first.com/articles/2022/11/09/reading-ctfiles-with-ctcore/).

# CTfile Documentation

The CTfile format is documented as follows:

- [CTFILE FORMATS 2020](https://discover.3ds.com/sites/default/files/2020-08/biovia_ctfileformats_2020.pdf)
- [CHEMICAL REPRESENTATION 2020](http://help.accelrysonline.com/insight/2020/content/pdf_files/bioviachemicalrepresentation.pdf)
- [BIOVIA Enhanced Stereochemical Representation](https://paperzz.com/doc/8466241/biovia-enhanced-stereochemical-representation)

# Test Suite

Runt the test suite:

```bash
cargo t --all
```

## Versions

CTcore is not yet stable. Patch versions never introduce breaking changes, but minor/major versions probably will.

## License and Copyright

CTcore is distributed under the terms of the MIT License. see [LICENSE](LICENSE) and [COPYRIGHT](COPYRIGHT) for details.