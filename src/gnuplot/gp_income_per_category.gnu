set terminal pngcairo size 1920,1080 enhanced font 'Inconsolata,12'
set output 'income_per_category.png'
set style data histogram
set style histogram clustered gap 1
set style fill transparent solid 0.4 noborder
set xtics nomirror scale 0 rotate by -45
set ytics add ('' 0) scale 0
set border 1
set grid ytics
set title "Income Per Category"
set ylabel "Amount"
plot "/tmp/ledgerplot/income_per_category.dat" using 2:xticlabels(1) notitle linecolor rgb "light-green", '' using 0:2:2 with labels font "Inconsolata,10" offset 0,0.5 textcolor linestyle 0 notitle
