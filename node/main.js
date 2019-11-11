const fs = require('fs');

productIdMap = {
    5202: true,
}

var main = {
    init() {
        fs.readFile('data.json', "utf8", (err, data) => {
            if (err) throw err
            var data = JSON.parse(data)
            // console.log(data)
            data = this.filterProducts(data)
            // console.log(data)
        })
    },
    filterProducts(data) {
        let dataType = typeof data
        if (dataType !== 'object') {
            return
        }
        if (Array.isArray(data)) {
            for (let i = 0; i < data.length; i++) {
                this.filterProducts(data[i])
            }
        } else {
            let p = data['simpleProductInfo']
            if (p) {
                if (p.productID && productIdMap[p.productID]) {
                    data['simpleProductInfo'] = null
                }
            } else {
                for (key in data) {
                    this.filterProducts(data[key])
                }
            }
        }
    }
}

main.init()