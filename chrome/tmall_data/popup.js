var main = {
    started: false
}

chrome.runtime.onMessage.addListener(function (request, sender) {
    if (request.action == "getSource") {
        chrome.storage.local.get(['products'], function (data) {
            let oldProducts = data['products']
            if (!oldProducts) {
                oldProducts = {}
            }

            let doc = $(request.source)
            let datas = doc.find('.J_TItems .item')

            datas.each(function () {
                let p = $(this)
                let id = p.attr('data-id')
                let img = p.find('.J_TGoldData img').attr('src')
                let name = p.find('.item-name.J_TGoldData').text()
                let price = p.find('.c-price').text()
                let shop = p.find('.c-price').text()
                let saleVolume = p.find('.sale-num').text()
                let evaluate = p.find('.rates span').text()

                oldProducts[id] = {
                    id: id,
                    name: name,
                    img: img,
                    price: price,
                    shop: shop,
                    saleVolume: saleVolume,
                    evaluate: evaluate,
                }
            })

            refreshData(oldProducts)

            chrome.storage.local.set({ 'products': oldProducts }, function () {
                chrome.tabs.query({ active: true, currentWindow: true }, function (tabs) {
                    chrome.tabs.executeScript(
                        tabs[0].id,
                        { code: 'window.location.href="' + doc.find('.ui-page-next').attr('href') + '";' });
                });
            })
            if (main.started) {
                setTimeout(startLoad, 3000*Math.round(1))
            }
        })
    }
})

function refreshData(oldProducts) {
    let a = 0
    for (k in oldProducts) {
        a++
    }
    $('#clearData').text('已收集' + a + '条数据,点击清理')
}
chrome.storage.local.get(['products'], function (data) {
    let oldProducts = data['products']
    if (!oldProducts) {
        oldProducts = {}
    }
    refreshData(oldProducts)
})

function startLoad() {
    chrome.tabs.query({ active: true, currentWindow: true }, function (tabs) {
        chrome.tabs.executeScript(
            tabs[0].id,
            { code: 'chrome.runtime.sendMessage({action:"getSource",source:document.body.innerHTML});' });
    });
};
loadData.onclick = function () {
    main.started = true
    startLoad()
}
stopLoadData.onclick = function () {
    main.started = false
}

clearData.onclick = function () {
    chrome.storage.local.set({ 'products': {} }, function () {
        $('#clearData').text('已收集' + 0 + '条数据,点击清理')
    })
};

const tableHeads = ['name', 'price', 'shop', 'saleVolume', 'evaluate']
const tableHeadMap = {
    'id': 'ID',
    'name': '商品名',
    'price': '价格',
    'shop': '店铺',
    'saleVolume': '30天销量',
    'evaluate': '评价数'
}
function getHead() {
    let v = ''
    for (i in tableHeads) {
        if (i > 0) {
            v += ','
        }
        v += '"' + tableHeadMap[tableHeads[i]] + '"'
    }
    return v
}
function getRow(o) {
    let v = '\n'
    for (i in tableHeads) {
        if (i > 0) {
            v += ','
        }
        v += '"' + (o[tableHeads[i]] + '').replace(/"/g, '').replace(/\n/g, '') + '"'
    }
    return v
}
downloadData.onclick = function (element) {
    let products = getHead()
    chrome.storage.local.get(['products'], function (data) {
        let oldProducts = data['products']
        if (!oldProducts) {
            oldProducts = {}
        }

        for (key in oldProducts) {
            let product = oldProducts[key]
            products += getRow(product)
        }

        var blob = new Blob([products], { type: "text/csv;charset=UTF-8" });
        var url = URL.createObjectURL(blob);
        chrome.downloads.download({
            url: url,
            filename: 'products.csv'
        });
    });
};