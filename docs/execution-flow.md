# Mie Scattering Monte Carlo: execution flow

本文件詳細說明目前 Rust workspace 中的模擬流程、物理量、統計量與輸出。
流程以 `simulation/src/main.rs` 為主，並依照目前程式行為描述；未實作的
功能會在限制章節明確列出。

## 1. 專案組成與執行方式

Workspace 包含三個 crate：

- `simulation`：Mie 計算、自由程抽樣、光子狀態更新與統計。
- `vector`：三維向量、Stokes 向量及 Mueller 矩陣所需的向量／矩陣運算。
- `draw`：把模擬統計畫成 PNG 圖。

在 repository 根目錄執行：

```powershell
cargo run -p simulation --release
```

開發時可省略 `--release`，但完整 photon run 的執行時間可能會長很多。執行
workspace 測試：

```powershell
cargo test --workspace
```

模擬參數集中於 `simulation/src/simulat_const.rs`。修改參數後，請確認所有數值
符合本文件的單位與模型限制。

目前設定要求執行 `500,000,000,000` 個 photon。完整執行可能非常耗時；
短測試應先降低 `PHOTON_CONST`，確認流程與輸出正常後再增加樣本數。

## 2. 輸入參數與單位慣例

目前模擬使用單一球形 polyethylene 粒子種類及均勻背景介質。

| 參數 | 程式欄位／常數 | 單位或意義 |
|---|---|---|
| 粒子折射率 | `POLYETHYLENE.m` | 無因次複數；目前輸入為粒子絕對折射率 |
| 粒子直徑 | `diameter_nm` | nm |
| 粒子莫耳濃度 | `molarity_mol_nm3` | mol/nm³ |
| 真空波長 | `WAVELENGH_NM` | nm |
| 背景介質折射率 | `N_MEDIUM` | 無因次 |
| 圓柱半徑 | `RADIUS_NM` | nm |
| 入射 photon 數 | `PHOTON_CONST` | 個 |
| 極角分箱數 | `FREQUENCY` | 個，涵蓋 0 到 π 的等寬 bin |

程式將輸入濃度乘以 Avogadro 常數得到粒子數密度：

`N [particles/nm³] = concentration [mol/nm³] × N_A [particles/mol]`

因此，這裡不應使用 mol/L 的 `10²⁴ nm³/L` 換算。截面以 nm² 表示，故
`mu_s`、`mu_t`、`mu_a` 都是 nm⁻¹；自由程、光子座標、圓柱半徑則是 nm。

Mie 計算使用背景介質中的相對折射率與波長：

`m_rel = m_particle / N_MEDIUM`

`lambda_medium = WAVELENGH_NM / N_MEDIUM`

`x = pi × diameter_nm / lambda_medium`

這些 Mie 參數均為無因次。程式在計算體積係數之前，會以球體體積
`pi d³ / 6` 檢查名義粒子體積分率；大於 1 時立即回報錯誤。這是幾何合理性
檢查，不代表所有小於 1 的濃度都已符合獨立粒子散射近似。

## 3. 啟動與一次性預計算

### 3.1 計算巨觀散射係數

`main` 首先呼叫 `find_solution_mu_sta`：

1. 依濃度計算粒子數密度。
2. 檢查名義體積分率。
3. 由 Mie 效率因子計算粒子截面：
   - `sigma_s = pi (d/2)² Q_sca`
   - `sigma_t = pi (d/2)² Q_ext`
   - `sigma_a = sigma_t - sigma_s`
4. 乘上粒子數密度得到：
   - `mu_s = N sigma_s`
   - `mu_t = N sigma_t`
   - `mu_a = N sigma_a`

自由程抽樣使用 `mu_t`。因此 `mu_t` 必須是與座標相同長度單位的倒數，
目前即 nm⁻¹。程式會列印 `mu_t` 及平均自由程 `1/mu_t`。

### 3.2 建立 Mie 角度矩陣與 CDF

`Particle::get_theta_vs_mueller_matrix` 會對每個 bin 中心計算角度振幅，
再建立對應的 Mueller 矩陣。角度格距為 `pi/FREQUENCY`，第 `i` 個 bin 的
中心角是：

`theta_i = (i + 1/2) pi / FREQUENCY`

`get_cdf_fn` 以 `S11(theta_i) sin(theta_i)` 作為角度機率權重並建立累積分布。
抽樣時用均勻亂數反查 CDF，取得一個離散極角 bin。該 bin 中心角用來查詢
Mueller 矩陣與更新 photon 方向。

這是以非偏振 `S11` 建立的極角提議分布。散射方位角 `psi` 目前獨立且均勻
分布於 `[0, 2pi)`。

## 4. Photon 初始狀態與主迴圈

主迴圈以 8 個 scoped worker 平行執行，工作以 `CUT` 個 photon 為一個 chunk。
worker 透過原子計數器動態領取下一個 chunk，減少固定分配下因各 photon 路徑長度
不同而造成的負載不均。每個 worker 持有自己的 RNG 與局部直方圖，chunk 完成後
才更新共享完成數；模擬資料及直方圖不需要跨執行緒加鎖。主執行緒週期性讀取
完成數以顯示進度，最後 join 所有 worker 並合併各自直方圖。

每個 photon 初始狀態如下：

- 位置 `start_location = (0, 0, 0)`。
- 傳播方向 `direction = (1, 0, 0)`。
- Stokes 向量 `(I,Q,U,V) = (1,0,0,0)`。
- photon 權重 `w = 1`。
- `scattered = false`。
- `last_plane_normal_v = (0,0,1)`，用來定義散射的局部座標參考方向。

每個 photon 重複呼叫 `move_a_path`：

- 回傳 `Some(photon)`：在介質內發生體積碰撞，執行一次 `collision_event`，
  再執行 Russian roulette 存活檢查。
- 回傳 `None`：透射離開圓柱，已將出射強度記入統計，該 photon 結束。
- roulette 淘汰：該 photon 結束，不會記錄出射貢獻。

## 5. 自由程與圓柱邊界

### 5.1 自由程抽樣

對均勻介質，每段候選自由程為：

`s = -ln(1 - U) / mu_t`, where `U` is uniform on `[0,1)`

程式把自由程乘以目前的單位方向，得到三維路徑向量。這是 Beer–Lambert
指數分布的自由程抽樣；均勻介質中每次碰撞後重新抽樣。

### 5.2 圓柱側壁判斷

容器沿 Z 軸無限延伸，因此是否仍在容器內只檢查 XY 平面：

`x² + y² <= RADIUS_NM²`

若路徑終點在圓柱內，整段路徑被走完，抵達位置成為體積碰撞點。若終點在外，
程式在 XY 平面解直線與圓的交點，更新至交點及剩餘路徑，再計算法線。
圓柱內向法線指向圓心。

在邊界交點附近，程式將 photon 沿內向法線推進 `RADIUS_NM × 10⁻⁹` nm，
避免下一次判斷仍停在側壁上。`z` 座標不因這個數值偏移而縮放。

### 5.3 Fresnel 反射或透射

程式依入射方向與邊界法線求入射角，並使用 `N_MEDIUM` 與 `N_BOUNDARY`
形成的 Snell/Fresnel 比值計算反射機率：

- 反射：以鏡面反射更新方向，重新正交化偏振參考法線；沿剩餘路徑繼續追蹤。
- 透射：使用 Snell 定律算折射方向，依方向的全域 X 分量計算出射極角，
  把 photon 的加權強度累積至對應角度 bin，並結束此 photon。

目前記錄的是圓柱邊界透射後、折射方向相對全域 +X 的角度。程式沒有再模擬
`N_BOUNDARY` 到空氣等後續界面。

## 6. 體積散射與偏振

抵達體積碰撞點後，`collision_event`：

1. 由 `S11` CDF 抽樣極角 bin，使用 bin 中心作 `theta`。
2. 均勻抽樣方位角 `psi`。
3. 將局部散射角轉換為全域傳播方向，並更新散射平面參考法線。
4. 以方位旋轉 Mueller 矩陣及該極角的 Mie Mueller 矩陣更新 Stokes 向量。
5. 將輸入 photon 標記為 `scattered = true`。

目前 Stokes 更新後以新強度正規化，並將相對強度變化放入 photon 權重。權重
更新形式為：

`w_new = w_old × (mu_s / mu_t) × intensity_ratio`

`mu_s/mu_t` 表示碰撞後成為散射事件的比例；`intensity_ratio` 補回這次
偏振散射取樣和 Stokes 強度變化所對應的強度因子。出射直方圖累積
`w × I`，而非未加權的 Stokes 強度。

## 7. Russian roulette

每次體積散射之後，若 photon 權重高於 `LIMIT_W_LINE`，繼續追蹤。若權重低於
門檻，則以 `1/M` 機率存活並把權重乘以 `M`；否則終止該 photon。存活機率與
權重補償使此剪枝在期望值上保持無偏。roulette 淘汰的 photon 不會貢獻出射統計。

## 8. 統計與立體角歸一化

程式使用兩組角度直方圖：

- `theta_log[0]`：沒有發生任何體積散射、直接透射的 ballistic photons。
- `theta_log[1]`：至少發生一次體積散射後透射的 photons。

每個透射 photon 將 `w × I` 累加到出射角 bin。模擬結束後，以總入射 photon
數及每個極角環帶的立體角歸一化：

`DeltaOmega_i = 2 pi [cos(i DeltaTheta) - cos((i+1) DeltaTheta)]`

`intensity_per_sr_i = count_i / (PHOTON_CONST × DeltaOmega_i)`

其結果表示「每個模擬入射 photon、每 sr 的加權出射強度」。環帶面積在 0°、
180° 附近較小，除以立體角後這些 bin 的統計雜訊可能特別明顯。空 bin 的
數值維持為零。

## 9. 進度及輸出

Mie 係數、Mueller 角度表及 CDF 都在計時器啟動前預先計算。因此進度 ETA 與
結束時列印的總耗時只涵蓋 photon 模擬迴圈，不包括這些預計算。

進度列以已完成 photon 的平均耗時估計 ETA，格式為 `HH:MM:SS`。啟動初期尚無
已完成 photon 時會顯示估算中；ETA 會隨實際平均速度更新。

模擬結束後會列印總耗時及粒子參數，並呼叫 `draw` 繪製 ballistic 和 scattered
兩條曲線。繪圖程式：

- 只用正強度值決定共同的 `log10` 顯示範圍。
- 以每 `0.5` 個 log decade 畫環形格線，可顯示負的 log 值。
- 以角度 bin 中心作為曲線角度。
- 對數軸下零或負值畫在徑向下限。
- 寫出 `picture/mie_scattering_high_dpi.png`（相對於執行時的工作目錄）。

兩條曲線使用同一個徑向範圍，便於比較 ballistic 與 scattered 分布。若圖中
沒有任何正強度，繪圖函式會回報錯誤，而不是產生空白的成功輸出。

## 10. 假設與目前限制

- 目前只有一種球形粒子，且背景介質均勻；混合多粒子種類或空間變化濃度尚未
  接入模擬。
- 邊界 Fresnel 反射機率取 s/p 偏振平均值；邊界反射及透射不更新完整 Stokes
  向量。因此體積散射有偏振追蹤，但界面偏振效應不完整。
- 散射極角由 `S11` 抽樣，方位角均勻；未直接從偏振相依的聯合
  `(theta, psi)` 分布抽樣。這需要與權重估計一起設計，不能只替換抽樣分布。
- 圓柱出射角分布包含邊界 Snell 折射及 Fresnel 選擇效果，不會一般性地等於
  單顆粒子的 Mie 相函數 `S11`。
- 目前輸出是角度分箱強度，不會輸出完整的 Stokes 角分布、透射／吸收能量平衡
  報表或碰撞歷史資料。
- 獨立散射粒子模型不涵蓋高體積分率下的粒子間近場耦合；體積分率檢查只是必要
  的基本合理性條件。

若要用 Mie 相函數驗證角度抽樣，應另設不帶圓柱界面折射的對照案例、採用相同
角度定義，並明確比較每 sr 歸一化後的分布。
