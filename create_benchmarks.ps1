$days="day01","day02","day03","day04","day05","day06","day07","day08","day09","day10","day11","day12","day13","day14","day15","day16","day17","day18","day19","day20","day21","day22","day23","day24"
$results = @{}
Foreach($d in $days)
{
    cd $d
    cargo clean
    cargo build
    cargo build --release
    $times = New-Object System.Collections.ArrayList
    1..100 | % {
        $t=Measure-Command{Invoke-Expression .\target\release\$d.exe}
        $times.Add($t) | Out-Null
    }
    $res = $times | Measure-Object -Property TotalMilliseconds -Minimum -Maximum -Average
    $results.Add($d, $res)
    cd ..
}

"Solution | Minimum Time | Average Time | Maximum Time"
"--- | ---: | ---: | ---:"
Foreach ($d in $days)
{
    $res = $results.Get_Item($d)
    "{0} | {1:0.00} ms | {2:0.00} ms | {3:0.00} ms" -f $d, $res.Minimum, $res.Average, $res.Maximum
}
