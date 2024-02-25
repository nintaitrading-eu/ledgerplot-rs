set terminal pngcairo size 1920,1080 enhanced font 'Inconsolata,10'
set output 'yearly_passive_income_vs_expenses.png'
set style data histogram
set style histogram clustered gap 1
set style fill transparent solid 0.4 noborder
set xtics nomirror scale 0 center
set ytics add ('' 0) scale 0
set border 1
set grid ytics
set title "Yearly Passive Income Versus Everyday Expenses"
set ylabel "Amount"
plot "/tmp/ledgerplot/ledgeroutput1.tmp" using 2:xticlabels(strftime('%Y', strptime('%Y-%m-%d', strcol(1)))) title "Passive income" linecolor rgb "green" with lines, "/tmp/ledgerplot/ledgeroutput2.tmp" using 2 title "Everyday expenses" linecolor rgb "orange" with lines, 25000 title "Max expenses" linecolor rgb "purple" linewidth 3
