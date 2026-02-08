

window.onload = function () {
    DataTable.defaults.column.orderSequence = ['asc', 'desc'];
    let reverse = new DataTable ( "#reverse-fusions", {
        paging: false,
        bInfo: false,
    });
    let forward = new DataTable ( "#forward-fusions", {
        paging: false,
        bInfo: false,
    });
}