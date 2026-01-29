$(document).ready(function () {
    DataTable.defaults.column.orderSequence = ['asc', 'desc'];
    $("#persona_table").DataTable({
        paging: false,
        bInfo: false,
    });
});