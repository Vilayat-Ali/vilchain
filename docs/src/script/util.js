const monthArr = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec'
];

const todayDate = `${new Date().getDate()} ${monthArr[new Date().getMonth()]} ${new Date().getFullYear()}`;

function formatAmount(amount){
    const formatter = Intl.NumberFormat('en', {
        notation: 'compact'
    });
    return formatter(amount);
}